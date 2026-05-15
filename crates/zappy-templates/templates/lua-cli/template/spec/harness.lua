do
    local spec_dir = debug.getinfo(1, 'S').source:match('@?(.*/)')
    if spec_dir and not spec_dir:match('^/') then
        spec_dir = require('lfs').currentdir() .. '/' .. spec_dir
    end
    local root = spec_dir:gsub('/spec/?$', '')
    package.path = root .. '/lua/?.lua;' .. root .. '/lua/?/init.lua;' .. package.path
end

for name in pairs(package.loaded) do
    if name == '__ZAPPY_PROJECT_NAME_SNAKE__' or name:match('^__ZAPPY_PROJECT_NAME_SNAKE__%.') then
        package.loaded[name] = nil
    end
end

local Test = {
    assert = require('luassert'),
    stub = require('luassert.stub'),
    lfs = require('lfs'),
    helpers = {},
    fixtures = {},
    og_globals = {},
    exit_handler = nil,
}

local inspect = require('inspect').inspect

local exit_stub

function Test.helpers.setup()
    Test.og_globals.arg = _G.arg
    _G.arg = {}
    exit_stub = Test.stub(_G.os, 'exit', function(...)
        if Test.exit_handler then
            return Test.exit_handler(...)
        end
    end)
end

function Test.helpers.teardown()
    Test.exit_handler = nil
    exit_stub:revert()
    _G.arg = Test.og_globals.arg
end

function Test.helpers.write_file(path, content)
    local file = io.open(path, 'w')
    if file ~= nil then
        file:write(content)
        file:close()
    end
end

function Test.helpers.dir_exists(path, exp)
    local res = Test.lfs.attributes(path, 'mode') == 'directory'
    if exp and exp == true then
        Test.assert.is_true(res)
    else
        Test.assert.is_false(res)
    end
end

function Test.helpers.array_contains(array, value)
    for _, val in ipairs(array) do
        if val == value then
            return true
        end
    end

    return false
end

function Test.helpers.tables_equal(t1, t2)
    if t1 == t2 then
        return true
    end

    if type(t1) ~= 'table' or type(t2) ~= 'table' then
        return false
    end

    for k, v in pairs(t1) do
        if not Test.helpers.tables_equal(v, t2[k]) then
            return false
        end
    end
    for k in pairs(t2) do
        if t1[k] == nil then
            return false
        end -- Extra keys in t2
    end

    return true
end

function Test.helpers.table_contains(tab, key)
    if tab and next(tab) then
        if tab[key] then
            return true
        end
    end

    return false
end

function Test.helpers.assert_tables_equal(t1, t2)
    local res = Test.helpers.tables_equal(t1, t2)
    if not res then
        print('Passed in:')
        print(inspect(t1))
        print('Expected:')
        print(inspect(t2))
    end

    Test.assert.is_true(res)
end

return Test
