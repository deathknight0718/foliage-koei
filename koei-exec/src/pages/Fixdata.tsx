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
import IconSave from "@mui/icons-material/Save";
import { IconButton } from "@mui/material";
import Layout from "../components/Layout";
import { TableArea } from "../components/table/TableArea";

// ----------------------------------------------------------------------------

function Trade() {
    return (
        <Layout className="flex flex-col gap-0 min-h-screen">
            <header className="flex items-center justify-between h-14 px-4 border-b border-gray-400">
                <h6 className="flex-1 text-subtitle1">共享剧本</h6>
                <IconButton size="small" color="primary" onClick={() => window.location.reload()}>
                    <IconSave />
                </IconButton>
            </header>
            <section className="w-full h-main bg-color-gray-100">
                <TableArea rows={[{ name: "测试" }]} />
            </section>
        </Layout>
    );
}

export default Trade;
