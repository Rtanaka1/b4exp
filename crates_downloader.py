#!/usr/bin/python3.8
import subprocess, os

def source_download(name, version):
    cmd = "curl -L 'https://crates.io/api/v1/crates/{crate}/{ver}/download' > /home/crates/{crate}-{ver}.tar.gz".format(crate=name, ver=version)
    subprocess.call(cmd, shell = True)
    print(f"crate: {name} downloaded!\n")

if __name__ == '__main__':
    path = "top3kscanned.txt"
    if not os.path.exists("/home/crates"):
        os.makedirs("/home/crates")
        print("/home/crates directory created.")
    else:
        print("The crates will be downloaded to /home/crates")
    with open(path, "r") as f:
        for line in f:
            l = line.rstrip().rsplit("-", 1)
            name = l[0]
            ver = l[1]
            source_download(name, ver)
    f.close()
    '''
    mesalink has broken archive on crates.io
    '''
    cmd = "git clone https://github.com/mesalock-linux/mesalink.git /home/crates/sources/mesalink-1.1.0"
    subprocess.call(cmd, shell = True)
