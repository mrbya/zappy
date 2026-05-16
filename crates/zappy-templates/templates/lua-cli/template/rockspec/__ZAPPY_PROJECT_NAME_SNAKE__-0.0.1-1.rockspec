package = '__ZAPPY_PROJECT_NAME_SNAKE__'
version = '0.0.1-1'
source = {
    url = '__REPO__',
    dir = '__ZAPPY_PROJECT_NAME_SNAKE__-v0.0.1',
}

description = {
    summary = '__DESCRIPTION__',
    detailed = [[
        __DESCRIPTION__,
    ]],
    homepage = '__REPO__',
    license = 'MIT',
}

dependencies = {
    'lua >= 5.1',
    'argparse >= 0.7.1',
}

build = {
    type = 'builtin',
    modules = {
        ['__ZAPPY_PROJECT_NAME_SNAKE__'] = 'lua/__ZAPPY_PROJECT_NAME_SNAKE__/init.lua',
    },
    install = {
        bin = {
            ['__ZAPPY_PROJECT_NAME_SNAKE__'] = 'bin/__ZAPPY_PROJECT_NAME_SNAKE__',
        },
    },
}
