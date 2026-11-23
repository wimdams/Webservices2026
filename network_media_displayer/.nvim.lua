-- Project-local nvim settings for network_media_displayer (STM32F746)
-- Auto-loaded by nvim when `exrc` is enabled (set exrc in init.lua or via :set exrc)

-- Set makeprg so :make works directly with this Makefile
vim.opt_local.makeprg = "make -j4 2>&1"

-- errorformat that understands GCC output  
vim.opt_local.errorformat = table.concat({
  "%f:%l:%c: %trror: %m",
  "%f:%l:%c: %tarning: %m",
  "%f:%l: %trror: %m",
  "%-GIn file included from %f:%l:,",
  "%-G%*[ ]from %f:%l:,",
  "%f:%l:%c: note: %m",
}, ",")

-- Swap to build output quickly
vim.keymap.set("n", "<localleader>b", "<cmd>make<CR>", { buffer = true, desc = "Build STM32" })
vim.keymap.set("n", "<localleader>c", "<cmd>make clean<CR>", { buffer = true, desc = "Clean" })
