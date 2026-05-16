local Api = {}

-- Global rock variables.
_G.__ZAPPY_PROJECT_NAME_SNAKE__ = {
    CMD = '__ZAPPY_PROJECT_NAME_SNAKE__',
    NAME = '__ZAPPY_PROJECT_NAME__',
    VERSION = '0.0.1',
    DESCRIPTION = '__DESCRIPTION__',
}

--- Dependencies
local argparse = require('argparse')

--- Argparse setup
local parser = argparse(_G.__ZAPPY_PROJECT_NAME_SNAKE__.CMD, _G.__ZAPPY_PROJECT_NAME_SNAKE__.DESCRIPTION)
-- Global args
parser:flag('-V --version', 'Display __ZAPPY_PROJECT_NAME_SNAKE__ version.')

--- greet command
local greet = parser:require_command(false):command('greet g')
greet:summary('Prints a greeting.')
greet:description('Prints a short greeting.')
greet:option('-n --name', 'Name to display in the greeting.'):argname('<NAME>'):args(1)

--- CLI arg parsing
Api.args = parser:parse()

--- Handles __ZAPPY_PROJECT_NAME_SNAKE__ cli version argument
---
--- Exits after printing app version.
---
local function version()
    if Api.args.version then
        print(_G.__ZAPPY_PROJECT_NAME_SNAKE__.CMD .. ' v' .. _G.__ZAPPY_PROJECT_NAME_SNAKE__.VERSION)
        os.exit(0)
    end
end

--- Handles __ZAPPY_PROJECT_NAME_SNAKE__ greet command
---
--- Prints a short greeting
---
local function run_greet()
    if Api.args.greet then
        print('Hello from __ZAPPY_PROJECT_NAME__!')
        if Api.args.name and Api.args.name ~= '' then
            print('Hi, ' .. Api.args.name .. '.')
        end
        os.exit(0)
    end
end

-- Runs __ZAPPY_PROJECT_NAME_SNAKE__
--
-- Resolves selected command and dispatches its handler.
--
function Api.run()
    version()
    run_greet()
    os.exit(0)
end

return Api
