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
import { isProd } from "@koei-exec/utils/node";
import { theme } from "@koei-exec/utils/theme";
import { CellStyleModule, ClientSideRowModelApiModule, ClientSideRowModelModule, LocaleModule, type LocaleText, RowSelectionModule, TextFilterModule, ValidationModule } from "ag-grid-community";
import { AgGridProvider, AgGridReact, type AgGridReactProps } from "ag-grid-react";

// ----------------------------------------------------------------------------

const modules = [ //
    ...(isProd() ? [] : [ValidationModule]),
    RowSelectionModule,
    ClientSideRowModelApiModule,
    ClientSideRowModelModule,
    TextFilterModule,
    CellStyleModule,
    LocaleModule,
];

const localeText: LocaleText = {
    filterOoo: "筛选...",
    equals: "等于",
    notEqual: "不等于",
    lessThan: "小于",
    greaterThan: "大于",
    lessThanOrEqual: "小于等于",
    greaterThanOrEqual: "大于等于",
    inRange: "区间",
    contains: "包含",
    notContains: "不包含",
    startsWith: "开头为",
    endsWith: "结尾为",
    blank: "为空",
    notBlank: "不为空",
    applyFilter: "应用",
    resetFilter: "重置",
    clearFilter: "清除",
    andCondition: "且",
    orCondition: "或",
};

// ----------------------------------------------------------------------------

export default function Table<T>({ tableRef, ...props }: AgGridReactProps<T> & { tableRef?: React.Ref<AgGridReact<T>> }) {
    const { tblTheme } = theme();
    return (
        <AgGridProvider modules={modules}>
            <AgGridReact ref={tableRef} {...props} localeText={localeText} theme={tblTheme} />
        </AgGridProvider>
    );
}
