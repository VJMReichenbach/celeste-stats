vim.api.nvim_create_autocmd("BufEnter", {
    pattern = "*.html.tera",
    callback = function()
        vim.bo.filetype = "htmldjango"
    end,
})
