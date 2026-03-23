#!/usr/bin/env python3
import json
import os
import re
import shutil
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


if __name__ == "__main__":
    unittest.main(verbosity=2)
