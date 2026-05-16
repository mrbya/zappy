--- @module "__ZAPPY_PROJECT_NAME_SNAKE__.commands"
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ commands module
--- Defines and loads plugin commands
---
local Commands = {}

local api = require("__ZAPPY_PROJECT_NAME_SNAKE__.api")

--- @class __ZAPPY_PROJECT_NAME_SNAKE__.commands.command
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ command
---
--- @field name string Command name
--- @field desc string Command description
--- @field mode string Neovim mode
--- @field callback function Command callback

--- @type table<__ZAPPY_PROJECT_NAME_SNAKE__.commands.command>
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ commands
---
Commands.commands = {
    --- Plugin commands
    --- e.g.:
    -- test = {
    --     name = '__ZAPPY_PROJECT_NAME_SNAKE__test',
    --     desc = '__ZAPPY_PROJECT_NAME_SNAKE__ plugin example command',
    --     mode = 'n',
    --     callback = function()
    --         print('Hello from __ZAPPY_PROJECT_NAME_SNAKE__!')
    --     end,
    -- }
}

--- Creates a user command
---
--- @param cmd __ZAPPY_PROJECT_NAME_SNAKE__.commands.command __ZAPPY_PROJECT_NAME_SNAKE__ command
---
function Commands.create_command(cmd)
    if cmd then
        vim.api.nvim_create_user_command(
            cmd.name,
            function ()
                cmd.callback()
            end,
            {
                desc = cmd.desc,
                force = true
            }
        )
    end
end

--- Loads __ZAPPY_PROJECT_NAME_SNAKE__ commands
---
--- @param config __ZAPPY_PROJECT_NAME_SNAKE__.config __ZAPPY_PROJECT_NAME_SNAKE__ config
---
function Commands.load(config)
    for _, command in pairs(Commands.commands) do
        Commands.create_command(command)
    end
end

return Commands
