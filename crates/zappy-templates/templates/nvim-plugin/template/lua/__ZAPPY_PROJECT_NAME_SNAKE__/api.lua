--- @module "__ZAPPY_PROJECT_NAME_SNAKE__.api"
---
--- Base file of __ZAPPY_PROJECT_NAME_SNAKE__ API
--- Contains the top-level implementation of __ZAPPY_PROJECT_NAME_SNAKE__ API
---
local Api = {
    --- API submodules
    --- e.g.:
    --- rendering = require('__ZAPPY_PROJECT_NAME_SNAKE__.api.rendering')
}

--- Loads API submodules
---
--- @param config __ZAPPY_PROJECT_NAME_SNAKE__.config __ZAPPY_PROJECT_NAME_SNAKE__ config table
---
function Api.load_submodules(config)
    --- Load API submodules
end

--- Loads __ZAPPY_PROJECT_NAME_SNAKE__ API
function Api.load(config)
    Api.load_submodules(config)
end

return Api
