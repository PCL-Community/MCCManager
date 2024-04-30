import { invoke } from "@tauri-apps/api";

const pathprefix = import.meta.env.VITE_CONFIG_PATH || "./app";

function readConfigFile(filename) {
    return new Promise((resolve, reject) => {
        invoke("read_config_file", { filename: `${pathprefix}/${filename}` })
            .then((result) => {
                resolve(result);
            })
            .catch((err) => {
                console.error(
                    `Error on read config file: file name \`${filename}\` error ${err}`
                );
                reject(err);
            });
    });
}

function updateConfigFile(filename, key, value) {
    return new Promise((resolve, reject) => {
        invoke("update_config_file", {
            filename: `${pathprefix}/${filename}`,
            key: key,
            value: value,
        })
            .then(resolve(null))
            .catch((err) => {
                console.error(
                    `Error on update config file: file name \`${filename}\`, key \`${key}\`, value \`${value}\` error ${err}`
                );
                reject(err);
            });
    });
}

const check = () => {
    return [pathprefix, import.meta.env.VITE_CONFIG_PATH];
};

export { readConfigFile, updateConfigFile, check };
