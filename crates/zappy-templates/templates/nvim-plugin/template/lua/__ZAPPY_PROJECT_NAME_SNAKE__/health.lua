return {
    --- @module "__ZAPPY_PROJECT_NAME_SNAKE__.health"
    ---
    --- Provides plugin health check logic for Neovim :checkhealth
    ---
    check = function()
        vim.health.start('__ZAPPY_PROJECT_NAME_SNAKE__')
        --- Plugin health check logic
    end
}
