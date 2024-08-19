console.clear()
const readline = require("readline")
const path = require("path")
const fs = require("fs")
let RBXDir = undefined

const RBXVersionsDir = path.join(process.env.APPDATA, "../Local/Roblox/Versions")
fs.readdirSync(RBXVersionsDir).forEach((ver) => {
    if(fs.existsSync(path.join(RBXVersionsDir, `${ver}/RobloxPlayerBeta.exe`))) {
        RBXDir = path.join(RBXVersionsDir, ver)
    }
})

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
})

rl.question("Use Unlocked FPS? (Y, N)", (answer) => {
    if(answer.toLowerCase() == "y") {
        const rl_fps = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            terminal: false
        })

        rl_fps.question("FPS: ", (fps) => {
            if(!fs.existsSync(path.join(RBXDir, "ClientSettings"))) {
                fs.mkdirSync(path.join(RBXDir, "ClientSettings"))
            }
            fs.writeFileSync(path.join(RBXDir, "ClientSettings/ClientAppSettings.json"), `{"DFIntTaskSchedulerTargetFps": ${fps},"FFlagGameBasicSettingsFramerateCap5": false,"FFlagTaskSchedulerLimitTargetFpsTo2402": "False"}`)
            process.exit()
        })
    } else {
        if(fs.existsSync(path.join(RBXDir, "ClientSettings"))) {
            fs.rm(path.join(RBXDir, "ClientSettings"), { recursive: true }, (err) => {
                if(err) throw err
            })
            setTimeout(() => {
                process.exit()
            }, 100);
        } else {
            console.log("ClientSettings not found.")
            process.exit()
        }
    }
})