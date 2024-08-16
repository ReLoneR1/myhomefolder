require "nvchad.mappings"

local map = vim.keymap.set

-- Default
map("n", ";", ":", { desc = "CMD enter command mode" })
map("i", "jk", "<ESC>", { desc = "exit insert mode" })

-- Russian keyboard
map("n", "ж", ":", { desc = "enter command mode" })
map("n", "ш", "i", { desc = "enter insert mode" })
map("n", "ф", "a", { desc = "enter insert mode" })
map("n", "Ш", "<S-i>", { desc = "enter insert mode" })
map("n", "Ф", "<S-a>", { desc = "enter insert mode" })
map("n", "м", "v", { desc = "enter visual mode" })
map("n", "М", "<S-v>", { desc = "enter string-visual mode" })
map("n", "з", "p", { desc = "paste" })
map("n", "З", "<S-p>", { desc = "paste" })
map("c", "й", "q", { desc = "quit" })
map("c", "ц", "w", { desc = "write" })
map("c", "ч", "x", { desc = "write and quit" })
map("v", "н", "y", { desc = "yank" })
map("v", "о", "j", { desc = "down" })
map("v", "л", "k", { desc = "up" })

-- Disabled
map("n", "<S-tab>", "")

--Term switch
map(
  "t",
  "<S-tab>",
  vim.api.nvim_replace_termcodes("<C-\\><C-N><C-w>h", true, true, true),
  { desc = "Escape terminal mode, Window left" }
)
map("n", "<S-tab>", "<C-w>li", { desc = "Window right" })

-- Vert move
map("n", "<C-d>", "<C-d>zz", { desc = "Dowl half page and centre cursor" })
map("n", "<C-u>", "<C-u>zz", { desc = "Up half page and centre cursor" })
map("n", "n", "nzz", { desc = "Next search and centre cursor" })
map("n", "N", "Nzz", { desc = "Previous search and centre cursor" })
