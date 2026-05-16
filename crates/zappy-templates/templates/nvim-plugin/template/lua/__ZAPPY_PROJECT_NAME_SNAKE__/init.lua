--- @module "__ZAPPY_PROJECT_NAME_SNAKE__"
---
--- Entry point of __ZAPPY_PROJECT_NAME_SNAKE__ plugin
---
local __ZAPPY_PROJECT_NAME_SNAKE__ = {
    autocommands = require('__ZAPPY_PROJECT_NAME_SNAKE__.autocommands'),
    api          = require('__ZAPPY_PROJECT_NAME_SNAKE__.api'),
    config       = require('__ZAPPY_PROJECT_NAME_SNAKE__.config'),
    commands     = require('__ZAPPY_PROJECT_NAME_SNAKE__.commands'),
    keymaps      = require('__ZAPPY_PROJECT_NAME_SNAKE__.keymaps'),
}

--- Parses user config and loads __ZAPPY_PROJECT_NAME_SNAKE__ API and modules
---
--- @param opts __ZAPPY_PROJECT_NAME_SNAKE__.config __ZAPPY_PROJECT_NAME_SNAKE__ config
---
function __ZAPPY_PROJECT_NAME_SNAKE__.setup(opts)
    __ZAPPY_PROJECT_NAME_SNAKE__.config = vim.tbl_deep_extend('force', __ZAPPY_PROJECT_NAME_SNAKE__.config, opts or {})

    __ZAPPY_PROJECT_NAME_SNAKE__.api.load(__ZAPPY_PROJECT_NAME_SNAKE__.config)
    __ZAPPY_PROJECT_NAME_SNAKE__.autocommands.load(__ZAPPY_PROJECT_NAME_SNAKE__.config)
    __ZAPPY_PROJECT_NAME_SNAKE__.commands.load(__ZAPPY_PROJECT_NAME_SNAKE__.config)
    __ZAPPY_PROJECT_NAME_SNAKE__.keymaps.load(__ZAPPY_PROJECT_NAME_SNAKE__.config)

    for _, module in ipairs(__ZAPPY_PROJECT_NAME_SNAKE__) do
        module.load(__ZAPPY_PROJECT_NAME_SNAKE__.config)
    end
end

return __ZAPPY_PROJECT_NAME_SNAKE__
