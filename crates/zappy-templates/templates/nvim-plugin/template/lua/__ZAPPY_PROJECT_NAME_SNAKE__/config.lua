--- @module "__ZAPPY_PROJECT_NAME_SNAKE__.config"

--- @class __ZAPPY_PROJECT_NAME_SNAKE__.config
---
--- Configuration table for __ZAPPY_PROJECT_NAME_SNAKE__ plugin
---
--- @field filetypes table Table setting file patterns for which to load __ZAPPY_PROJECT_NAME_SNAKE__
--- @field keymaps table Table containing interactive mode keymaps
---
local config = {
    filetypes = { '*' },

    --- @enum __ZAPPY_PROJECT_NAME_SNAKE__.config.keymap
    ---
    --- Command keymaps
    ---
    keymaps = {
        --- Plugin keymaps (command = 'keymap')
        --- e.g.:
        --- test = '<leader>t',
    },
}

return config
