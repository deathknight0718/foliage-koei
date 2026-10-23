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
import type { ColDef, GetRowIdParams } from "ag-grid-community";
import { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { useColumnText } from "./column/ColumnText";
import Table from "./Table";

// ----------------------------------------------------------------------------

interface Area {
    name: string;
}

// ----------------------------------------------------------------------------

function useColumns(): ColDef<Area>[] {
    const { t } = useTranslation("cols");
    const c1 = useColumnText<Area>({ flex: 2.0, path: "name", head: t("name") });
    return [c1] as const;
}

// ----------------------------------------------------------------------------

function LoadingOverlay({ message }: { message: string }) {
    return (
        <div className="flex flex-col items-center justify-center h-full gap-2">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-main" />
            <span className="text-body2 pt-2">{message}</span>
        </div>
    );
}

// ----------------------------------------------------------------------------

export function TableArea({ rows }: { rows: Area[] }) {
    const columns = useColumns();
    const getRowId = useCallback(({ data }: GetRowIdParams<Area>) => String(data!.name), []);
    return (
        <Table
            suppressCellFocus
            loading={false}
            getRowId={getRowId}
            rowData={rows}
            columnDefs={columns}
            loadingOverlayComponent={LoadingOverlay}
            loadingOverlayComponentParams={{ message: "" }}
        />
    );
}
