/*
 * MIT License
 *
 * Copyright (c) 2025 Foliage Develop Team
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
import { zhCN } from "@mui/material/locale";
import type { Breakpoint, Theme as MuiTheme } from "@mui/material/styles";
import { useTheme as useMuiTheme } from "@mui/material/styles";
import { createTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import type { Theme as AgTable, ThemeDefaultParams } from "ag-grid-community";
import { colorSchemeDark, themeBalham } from "ag-grid-community";

// ----------------------------------------------------------------------------

const MUI_THEME = createTheme({
    cssVariables: true,
    palette: { mode: "dark" },
    typography: { fontFamily: "Sarasa, monospace" },
    shape: { borderRadius: 0 },
    breakpoints: { values: { xs: 0, sm: 768, md: 1024, lg: 1280, xl: 1440 } },
    components: {
        MuiButton: { defaultProps: { size: "small" }, styleOverrides: { root: { fontFamily: "Sarasa, monospace" } } },
        MuiTextField: { defaultProps: { size: "small" } },
        MuiIconButton: { defaultProps: { size: "small", onMouseDown: (event) => event.preventDefault() } },
        MuiChip: { defaultProps: { size: "small" } },
        MuiTableCell: { defaultProps: { size: "small" } },
        MuiCheckbox: { defaultProps: { size: "small" } },
        MuiRadio: { defaultProps: { size: "small" } },
        MuiSwitch: { defaultProps: { size: "small" } },
        MuiFormControl: { defaultProps: { size: "small" } },
        MuiInputBase: { defaultProps: { size: "small" } },
        MuiSelect: { defaultProps: { size: "small" } },
    },
}, zhCN);

const TBL_THEME = themeBalham.withParams({
    fontFamily: "Sarasa, monospace",
    headerFontFamily: "Sarasa, monospace",
    wrapperBorder: false,
}).withPart(colorSchemeDark);

const THEME = {
    muiTheme: MUI_THEME, //
    tblTheme: TBL_THEME,
    fontFamily: "Sarasa, monospace",
    fontBoldFamily: "Sarasa Bold, monospace",
    fontSemiboldFamily: "Sarasa Semi, monospace",
} as Theme;

// ----------------------------------------------------------------------------

export interface Theme {
    muiTheme: MuiTheme;
    tblTheme: AgTable<ThemeDefaultParams>;
    fontFamily: string;
    fontBoldFamily: string;
    fontSemiboldFamily: string;
}

export function theme() {
    return THEME;
}
export function useMediaUp(point: Breakpoint) {
    const theme = useMuiTheme();
    return useMediaQuery(theme.breakpoints.up(point));
}

/** 颜色: CSS hex → [r, g, b, a] 归一化 0~1 */
export function rgba(hex: string): [number, number, number, number] {
    const h = hex.replace("#", "");
    const r = parseInt(h.substring(0, 2), 16) / 255;
    const g = parseInt(h.substring(2, 4), 16) / 255;
    const b = parseInt(h.substring(4, 6), 16) / 255;
    return [r, g, b, 1.0];
}

export function rgbc(color: string, alpha: number) {
    const [r, g, b] = rgba(color);
    return `rgba(${Math.round(r * 255)}, ${Math.round(g * 255)}, ${Math.round(b * 255)}, ${alpha})`;
}
