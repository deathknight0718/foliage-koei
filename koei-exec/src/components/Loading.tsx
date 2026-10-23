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

import { useTranslation } from "react-i18next";
import { blue, deep_orange, gray, green, orange, purple, red } from "../utils/palette";

export default function Component() {
    const { t } = useTranslation("init");
    return (
        <main className="fixed inset-0 flex flex-col items-center justify-center gap-8">
            <svg className="loading-ip" viewBox="0 0 256 128" width="256px" height="128px" xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <linearGradient id="grad1" x1="0" y1="0" x2="1" y2="0">
                        <stop offset="0%" stopColor={green.n400} />
                        <stop offset="33%" stopColor={orange.n400} />
                        <stop offset="67%" stopColor={deep_orange.n400} />
                        <stop offset="100%" stopColor={red.n400} />
                    </linearGradient>
                    <linearGradient id="grad2" x1="1" y1="0" x2="0" y2="0">
                        <stop offset="0%" stopColor={red.n400} />
                        <stop offset="33%" stopColor={purple.n400} />
                        <stop offset="67%" stopColor={blue.n400} />
                        <stop offset="100%" stopColor={green.n400} />
                    </linearGradient>
                </defs>
                <g fill="none" strokeLinecap="round" strokeWidth="16">
                    <g className="loading-track" stroke={gray.n300}>
                        <path d="M8,64s0-56,60-56,60,112,120,112,60-56,60-56" />
                        <path d="M248,64s0-56-60-56-60,112-120,112S8,64,8,64" />
                    </g>
                    <g strokeDasharray="180 656">
                        <path className="loading-worm1" stroke="url(#grad1)" strokeDashoffset="0" d="M8,64s0-56,60-56,60,112,120,112,60-56,60-56" />
                        <path className="loading-worm2" stroke="url(#grad2)" strokeDashoffset="358" d="M248,64s0-56-60-56-60,112-120,112S8,64,8,64" />
                    </g>
                </g>
            </svg>
            <p>{t("loading")}</p>
        </main>
    );
}
