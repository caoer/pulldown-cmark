#!/usr/bin/env python3
"""GT-v2.2 overlay verifier.

Materializes the v2.2 consumer view (base pack with amended expected files
substituted, v2.2 layer over v2.1 layer over base) in a temp dir and runs
the FROZEN tools/verify.py on it, byte-for-byte unmodified — same
invariants, same code, zero violations required. Exits with verify.py's
exit code.

View = fixtures (corpus/, adversarial/) + MANIFEST.tsv symlinked from the
base pack; ground-truth/ = base expected files, overridden by
amendments/v2.1/ground-truth/ where present, overridden by
amendments/v2.2/ground-truth/ where present (later layer wins); tools/
verify.py copied (copied, not symlinked: it derives its ROOT from its own
resolved path).
"""
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
AMEND = HERE.parent                       # amendments/v2.2
GT2 = AMEND.parent.parent                 # tests/gt-v2
LAYERS = (GT2 / "amendments" / "v2.1", AMEND)   # earlier -> later


def main() -> None:
    with tempfile.TemporaryDirectory(prefix="gt-v2.2-view-") as td:
        view = Path(td)
        for d in ("corpus", "adversarial"):
            (view / d).symlink_to(GT2 / d)
        (view / "MANIFEST.tsv").symlink_to(GT2 / "MANIFEST.tsv")

        gt = view / "ground-truth"
        gt.mkdir()
        for p in sorted((GT2 / "ground-truth").glob("*.expected.json")):
            src = p
            for layer in LAYERS:
                over = layer / "ground-truth" / p.name
                if over.is_file():
                    src = over
            (gt / p.name).symlink_to(src)

        tools = view / "tools"
        tools.mkdir()
        shutil.copyfile(GT2 / "tools" / "verify.py", tools / "verify.py")

        proc = subprocess.run([sys.executable, str(tools / "verify.py")])
        sys.exit(proc.returncode)


if __name__ == "__main__":
    main()
