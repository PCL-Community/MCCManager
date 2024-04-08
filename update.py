from json import load, dump

with open(r"src-tauri\\tauri.conf.json", "r", encoding="utf-8") as tcj:
    tcj_all = load(tcj)
    version_old:str = tcj_all["package"]["version"]

major, minor, patch = version_old.split(".")
major, minor, patch = int(major), int(minor), int(patch)
version_new = input(f"新的版本号  [{major}.{minor}.{patch+1}]: ") or f"{major}.{minor}.{patch+1}"

# Start build-ci.yml #
with open(r".github\\workflows\\build-ci.yml", "r", encoding="utf-8") as ci:
    data = ci.read()
data = data.replace(f"MCCManager {version_old}", f"MCCManager {version_new}")
data = data.replace(f"\mccmanager_{version_old}", f"\mccmanager_{version_new}")
with open(r".github\\workflows\\build-ci.yml", "w", encoding="utf-8") as ci:
    ci.write(data)
#  End build-ci.yml  #

# Start package.json #
with open("package.json", "r", encoding="utf-8") as pj:
    data = load(pj)
data["version"] = version_new
with open("package.json", "w", encoding="utf-8") as pj:
    dump(data, pj, ensure_ascii=False, indent=4)
#  End package.json  #

# Start package-lock.json #
with open("package-lock.json", "r", encoding="utf-8") as pj:
    data = load(pj)
data["version"] = version_new
with open("package-lock.json", "w", encoding="utf-8") as pj:
    dump(data, pj, ensure_ascii=False, indent=4)
#  End package-lock.json  #

# Start tauri.conf.json #
tcj_all["package"]["version"] = version_new
with open(r"src-tauri\\tauri.conf.json", "w", encoding="utf-8") as tcj:
    dump(tcj_all, tcj, ensure_ascii=False, indent=4)
#  End tauri.conf.json  #

# Start Cargo.toml #
with open(r"src-tauri\\Cargo.toml", "r", encoding="utf-8") as ct:
    data = ct.read()
data = data.replace(f"version = \"{version_old}\"", f"version = \"{version_new}\"")
with open(r"src-tauri\\Cargo.toml", "w", encoding="utf-8") as ct:
    ct.write(data)
#  End Cargo.toml  #

print(f"已更新版本号至 [{version_new}] 于 5 文件")
