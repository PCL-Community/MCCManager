from json import load, dump

with open(r"src-tauri\\tauri.conf.json", "r", encoding="utf-8") as tcj:
    tcj_all = load(tcj)
    version_now:str = tcj_all["package"]["version"]

version_now_major = int(version_now.split(".")[0])
version_now_minor = int(version_now.split(".")[1])
version_now_patch = int(version_now.split(".")[2])

version_new = input(f"新的版本号  [{version_now_major}.{version_now_minor}.{version_now_patch+1}]: ") or f"{version_now_major}.{version_now_minor}.{version_now_patch+1}"

# Start build-ci.yml #
with open(r".github\\workflows\\build-ci.yml", "r", encoding="utf-8") as ci:
    data = ci.read()
data = data.replace(f"MCCManager {version_now}", f"MCCManager {version_new}")
data = data.replace(f"\mccmanager_{version_now}", f"\mccmanager_{version_new}")
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

print(f"已更新版本号至 [{version_new}] 于 4 文件")
