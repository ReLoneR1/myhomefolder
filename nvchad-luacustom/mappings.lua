---@type MappingsTable
local M = {}

M.general = {
  n = {
    [";"] = { ":", "enter command mode", opts = { nowait = true } },
    ["ж"] = { ":", "enter command mode", opts = { nowait = true } },
    ["ш"] = { "i", "enter insert mode", opts = { nowait = true } },
    ["ф"] = { "a", "enter insert mode", opts = { nowait = true } },
    ["Ш"] = { "<S-i>", "enter insert mode", opts = { nowait = true } },
    ["Ф"] = { "<S-a>", "enter insert mode", opts = { nowait = true } },
    ["м"] = { "v", "enter visual mode", opts = { nowait = true } },
    ["М"] = { "<S-v>", "enter string-visual mode", opts = { nowait = true } },
    ["з"] = { "p", "paste" },

    --  format with conform
    ["<leader>fm"] = {
      function()
        require("conform").format()
      end,
      "formatting",
    }

  },

  c = {
    ["й"] = { "q", "quit"},
    ["ц"] = { "w", "write"},
    ["ч"] = { "x", "write and quit"}
  },

  v = {
    [">"] = { ">gv", "indent"},
    ["н"] = { "y", "yank"},
    ["о"] = { "j", "down"},
    ["л"] = { "k", "up"},
  },
}

-- more keybinds!
M.disabled = {
  n = {
      ["<S-tab>"] = "",
    -- ["<C-l>"] = ""
  }
}

M.termswitch = {
  t = {
      ["<S-tab>"] = { vim.api.nvim_replace_termcodes("<C-\\><C-N><C-w>h", true, true, true), "Escape terminal mode, Window left" }
  },
  n = {
      ["<S-tab>"] = { "<C-w>l", "Window right"}
  }
}

M.vertmove = {
  n = {
      ["<C-d>"] = { "<C-d>zz", "Down half page and centre cursor"},
      ["<C-u>"] = { "<C-u>zz", "Up half page and centre cursor"},
      ["n"] = {"nzz", "Next search options and centre cursor"},
      ["N"] = {"Nzz", "Next search options and centre cursor"}
  }
}


return M
