local test = require('spec.harness')

local function reload___ZAPPY_PROJECT_NAME_SNAKE__()
    package.loaded['__ZAPPY_PROJECT_NAME_SNAKE__'] = nil
    return require('__ZAPPY_PROJECT_NAME_SNAKE__')
end

describe('__ZAPPY_PROJECT_NAME_SNAKE__ top-level API tests', function()
    local print_stub

    before_each(function()
        test.helpers.setup()
        print_stub = test.stub(_G, 'print')
        test.exit_handler = function()
            error('exit')
        end
    end)

    after_each(function()
        print_stub:revert()
        test.helpers.teardown()
        test.exit_handler = nil
    end)

    it('loads __ZAPPY_PROJECT_NAME_SNAKE__ without crashing', function()
        local __ZAPPY_PROJECT_NAME_SNAKE__ = reload___ZAPPY_PROJECT_NAME_SNAKE__()
        test.assert.is_truthy(__ZAPPY_PROJECT_NAME_SNAKE__)
        test.assert.is_table(__ZAPPY_PROJECT_NAME_SNAKE__.api)
    end)

    it('prints version and exits when requested', function()
        local __ZAPPY_PROJECT_NAME_SNAKE__ = reload___ZAPPY_PROJECT_NAME_SNAKE__()
        local og_args = __ZAPPY_PROJECT_NAME_SNAKE__.api.args
        __ZAPPY_PROJECT_NAME_SNAKE__.api.args = { version = true }

        local ok, err = pcall(function()
            __ZAPPY_PROJECT_NAME_SNAKE__.api.run()
        end)

        test.assert.is_false(ok)
        test.assert.is_truthy(err:match('exit'))
        test.assert
            .stub(print_stub)
            .was_called_with(_G.__ZAPPY_PROJECT_NAME_SNAKE__.CMD .. ' v' .. _G.__ZAPPY_PROJECT_NAME_SNAKE__.VERSION)
        __ZAPPY_PROJECT_NAME_SNAKE__.api.args = og_args
    end)

    it('prints greeting and exists when requested', function()
        local __ZAPPY_PROJECT_NAME_SNAKE__ = reload___ZAPPY_PROJECT_NAME_SNAKE__()
        local og_args = __ZAPPY_PROJECT_NAME_SNAKE__.api.args
        __ZAPPY_PROJECT_NAME_SNAKE__.api.args = { greet = true, name = 'Bruce Lee' }

        local ok, err = pcall(function()
            __ZAPPY_PROJECT_NAME_SNAKE__.api.run()
        end)

        test.assert.is_false(ok)
        test.assert.is_truthy(err:match('exit'))
        test.assert.stub(print_stub).was_called_with('Hi, Bruce Lee.')
        __ZAPPY_PROJECT_NAME_SNAKE__.api.args = og_args
    end)
end)
