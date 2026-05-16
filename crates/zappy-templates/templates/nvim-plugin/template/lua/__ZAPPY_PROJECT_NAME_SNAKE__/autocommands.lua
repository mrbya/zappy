--- @module "__ZAPPY_PROJECT_NAME_SNAKE__.autocommands"
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ plugin module holding its autocommands
---
local Autocommands = {}

local api = require('__ZAPPY_PROJECT_NAME_SNAKE__.api')

--- @class __ZAPPY_PROJECT_NAME_SNAKE__.autocommands.autocommand
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ autocommand
---
--- @field events table<string> Events triggering autocommand
--- @field callback function Autocommand callback

--- @type table<__ZAPPY_PROJECT_NAME_SNAKE__.autocommands.autocommand>
---
--- __ZAPPY_PROJECT_NAME_SNAKE__ autocommands
---
Autocommands.autocommands = {
    --- Plugin autocommands
    --- e.g.:
    -- {
    --     events = { 'BufEnter' },
    --     callback = function()
    --         print('Hello from __ZAPPY_PROJECT_NAME_SNAKE__!')
    --     end
    -- },
}

--- Function to load autocommands
---
--- @param config __ZAPPY_PROJECT_NAME_SNAKE__.config
---
function Autocommands.load(config)
    for _, autocommand in ipairs(Autocommands.autocommands) do
        vim.api.nvim_create_autocmd(
            autocommand.events,
            {
                pattern = config.filetypes,
                callback = function()
                    autocommand.callback()
                end
            }
        )
    end
end

return Autocommands
