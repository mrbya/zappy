--- @module "__ZAPPY_PROJECT_NAME_SNAKE__.keymaps"
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ keymaps module
--- Loads keymaps for plugin commands based on its config
---
local Keymaps = {}

local commands = require('__ZAPPY_PROJECT_NAME_SNAKE__.commands').commands

--- Create a keymap
---
--- @param cmd __ZAPPY_PROJECT_NAME_SNAKE__.commands.command __ZAPPY_PROJECT_NAME_SNAKE__ command
--- @param keymap __ZAPPY_PROJECT_NAME_SNAKE__.config.keymap __ZAPPY_PROJECT_NAME_SNAKE__ keymap
---
function Keymaps.create_keymap(cmd, keymap)
    if keymap and cmd then
        vim.keymap.set(
            cmd.mode,
            keymap,
            function()
                cmd.callback(keymap)
            end,
            {
                noremap = true,
                silent = true,
                desc = cmd.desc,
            }
        )
    end
end

--- Load __ZAPPY_PROJECT_NAME_SNAKE__ keymaps
---
--- @param config __ZAPPY_PROJECT_NAME_SNAKE__.config
---
function Keymaps.load(config)
    local keymaps = config.keymaps

    for command, keymap in pairs(keymaps) do
        Keymaps.create_keymap(commands[command], keymap)
    end
end

return Keymaps
