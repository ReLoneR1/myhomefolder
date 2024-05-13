local overrides = require("custom.configs.overrides")

---@type NvPluginSpec[]
local plugins = {

    {
        "vhyrro/luarocks.nvim",
        priority = 1000,
        config = true,
        opts = {
          luarocks_build_args = {
            "--with-lua-include=/usr/include",
    },
    },
    },

    {
        "nvim-neorg/neorg",
        dependencies = { "luarocks.nvim" },
        -- put any other flags you wanted to pass to lazy here!
        lazy = false,
        version = "*",
        config = function()
            require("neorg").setup({
                -- put any of your previous config here
              load = {
                  ["core.defaults"] = {},
                  ["core.concealer"] = {},
              }
            })
        end,
    },

  -- Override plugin definition options_info
  {
    "neovim/nvim-lspconfig",
    config = function()
      require "plugins.configs.lspconfig"
      require "custom.configs.lspconfig"
    end, -- Override to setup mason-lspconfig
  },

  -- override plugin configs
  {
    "williamboman/mason.nvim",
    opts = overrides.mason
  },

  {
    "nvim-treesitter/nvim-treesitter",
    opts = overrides.treesitter,
  },

  {
    "nvim-tree/nvim-tree.lua",
    opts = overrides.nvimtree,
  },

  -- Install a plugin
  {
    "max397574/better-escape.nvim",
    event = "InsertEnter",
    config = function()
      require("better_escape").setup()
    end,
  },

  {
    "stevearc/conform.nvim",
    --  for users those who want auto-save conform + lazyloading!
    -- event = "BufWritePre"
    config = function()
      require "custom.configs.conform"
    end,
  },

  -- To make a plugin not be loaded
  -- {
  --   "NvChad/nvim-colorizer.lua",
  --   enabled = false
  -- },

  -- All NvChad plugins are lazy-loaded by default
  -- For a plugin to be loaded, you will need to set either `ft`, `cmd`, `keys`, `event`, or set `lazy = false`
  -- If you want a plugin to load on startup, add `lazy = false` to a plugin spec, for example
  -- {
  --   "mg979/vim-visual-multi",
  --   lazy = false,
  -- }
}

return plugins
