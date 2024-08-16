-- https://github.com/NvChad/ui/blob/v2.5/lua/nvconfig.lua

---@type ChadrcConfig
local M = {}

M.base46 = {
	theme = "everforest",

	hl_override = {
	  Comment = { italic = true },
	  ["@comment"] = { italic = true },
	},

  transparency = true,
}

M.ui = {
  statusline = {
    theme = "minimal",
    separator_style = "block",
  },
}

return M
