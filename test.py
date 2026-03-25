#!/usr/bin/env python3
import json
import os
import re
import shutil
import subprocess
import tempfile
import unittest


def make_slug(cmd):
    slug = re.sub(r'[^a-zA-Z0-9]+', '-', cmd).strip('-')
    return slug[:60]


def run_setup(cmd, cwd):
    """Replicates plog's setup: slug, dirs, info.json, gitignore."""
    slug = make_slug(cmd)
    d = os.path.join(cwd, "plogs", slug)
    os.makedirs(d, exist_ok=True)

    gitignore = os.path.join(cwd, ".gitignore")
    if os.path.exists(gitignore):
        with open(gitignore, "r+") as f:
            lines = f.read().splitlines()
            if "plogs" not in lines:
                f.write("" if lines and lines[-1] == "" else "\n")
                f.write("plogs\n")

    with open(os.path.join(d, "info.json"), "w") as f:
        json.dump({"command": cmd, "started": "2026-01-01T00:00:00+00:00"}, f)

    return d



def read_info(d):
    with open(os.path.join(d, "info.json")) as f:
        return json.load(f)


class TestPlog(unittest.TestCase):

    def setUp(self):
        self.tmp = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.tmp)

    # ── slug ──────────────────────────────────────────────────────────────────

    def test_slug_basic(self):
        self.assertEqual(make_slug("npm run build"), "npm-run-build")

    def test_slug_special_chars(self):
        self.assertEqual(make_slug("sh -c 'exit 42'"), "sh-c-exit-42")

    def test_slug_max_length(self):
        self.assertLessEqual(len(make_slug("a" * 100)), 60)

    def test_slug_no_leading_trailing_dash(self):
        slug = make_slug("  npm run build  ")
        self.assertFalse(slug.startswith("-"))
        self.assertFalse(slug.endswith("-"))

    # ── directory structure ───────────────────────────────────────────────────

    def test_creates_plogs_dir(self):
        run_setup("echo test", self.tmp)
        self.assertTrue(os.path.isdir(os.path.join(self.tmp, "plogs")))

    def test_creates_command_subdir(self):
        d = run_setup("echo test", self.tmp)
        self.assertTrue(os.path.isdir(d))

    def test_separate_dirs_per_command(self):
        run_setup("make test", self.tmp)
        run_setup("make build", self.tmp)
        self.assertTrue(os.path.isdir(os.path.join(self.tmp, "plogs", "make-test")))
        self.assertTrue(os.path.isdir(os.path.join(self.tmp, "plogs", "make-build")))

    # ── info.json ─────────────────────────────────────────────────────────────

    def test_info_json_exists(self):
        d = run_setup("echo test", self.tmp)
        self.assertTrue(os.path.exists(os.path.join(d, "info.json")))

    def test_info_json_command_field(self):
        d = run_setup("npm run build", self.tmp)
        self.assertEqual(read_info(d)["command"], "npm run build")

    def test_info_json_started_present(self):
        d = run_setup("echo test", self.tmp)
        self.assertIn("started", read_info(d))

    # ── gitignore ─────────────────────────────────────────────────────────────

    def test_gitignore_appended(self):
        open(os.path.join(self.tmp, ".gitignore"), "w").close()
        run_setup("echo test", self.tmp)
        with open(os.path.join(self.tmp, ".gitignore")) as f:
            self.assertIn("plogs", f.read().splitlines())

    def test_gitignore_not_duplicated(self):
        open(os.path.join(self.tmp, ".gitignore"), "w").close()
        run_setup("echo test", self.tmp)
        run_setup("echo test", self.tmp)
        with open(os.path.join(self.tmp, ".gitignore")) as f:
            self.assertEqual(f.read().splitlines().count("plogs"), 1)

    def test_no_gitignore_no_error(self):
        run_setup("echo test", self.tmp)

    # ── flags: -h / --help / --list ───────────────────────────────────────────

    def _plog(self, *args, **kwargs):
        plog = os.path.join(os.path.dirname(__file__), "plog")
        return subprocess.run([plog, *args], cwd=self.tmp,
                              capture_output=True, text=True, **kwargs)

    def test_help_short_exits_zero(self):
        self.assertEqual(self._plog("-h").returncode, 0)

    def test_help_long_exits_zero(self):
        self.assertEqual(self._plog("--help").returncode, 0)

    def test_help_short_shows_usage(self):
        self.assertIn("Usage:", self._plog("-h").stdout)

    def test_help_long_shows_usage(self):
        self.assertIn("Usage:", self._plog("--help").stdout)

    def test_help_does_not_create_plogs_dir(self):
        self._plog("-h")
        self.assertFalse(os.path.exists(os.path.join(self.tmp, "plogs")))

    def test_list_no_plogs_dir(self):
        result = self._plog("--list")
        self.assertEqual(result.returncode, 0)
        self.assertIn("No plogs yet.", result.stdout)

    def test_list_empty_plogs_dir(self):
        os.makedirs(os.path.join(self.tmp, "plogs"))
        result = self._plog("--list")
        self.assertIn("No plogs yet.", result.stdout)

    def test_list_shows_entries(self):
        run_setup("npm run build", self.tmp)
        open(os.path.join(self.tmp, "plogs", "npm-run-build", "output.log"), "w").close()
        result = self._plog("--list")
        self.assertIn("plogs/npm-run-build/output.log", result.stdout)
        self.assertIn("npm run build", result.stdout)

    def test_list_does_not_create_plogs_dir(self):
        self._plog("--list")
        self.assertFalse(os.path.exists(os.path.join(self.tmp, "plogs")))

    def test_list_no_plogs_yet_has_no_help_text(self):
        result = self._plog("--list")
        self.assertNotIn("Usage:", result.stdout)

    # ── output.log recreation ─────────────────────────────────────────────────

    # ── output.log recreation ─────────────────────────────────────────────────

    def test_output_log_recreated_on_rerun(self):
        plog = os.path.join(os.path.dirname(__file__), "plog")
        log = os.path.join(self.tmp, "plogs", "echo-first", "output.log")
        subprocess.run([plog, "echo", "first"], cwd=self.tmp, check=True, capture_output=True)
        first_size = os.path.getsize(log)
        subprocess.run([plog, "echo", "first"], cwd=self.tmp, check=True, capture_output=True)
        second_size = os.path.getsize(log)
        self.assertEqual(first_size, second_size,
                         "output.log should be recreated, not appended to")


if __name__ == "__main__":
    unittest.main(verbosity=2)
