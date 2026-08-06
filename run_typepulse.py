import os, subprocess

def run_typepulse(directory):
    print(f"processing: {directory}")
    os.chdir(directory)
    # subprocess.call("rustup override set nightly-2023-06-02-x86_64-unknown-linux-gnu", shell=True)
    subprocess.call("cargo typepulse -j 16 > report.txt 2>&1", shell=True)
    print(f"Completed `cargo typepulse` in {directory}")

if __name__ == "__main__":
    crates_base_path = "/home/crates/sources/"
    packages = [
        os.path.join(crates_base_path, d) for d in os.listdir(crates_base_path)
        if os.path.isdir(os.path.join(crates_base_path, d))
    ]

    for p in packages:
        run_typepulse(p)
