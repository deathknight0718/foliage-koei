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
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

// ----------------------------------------------------------------------------
// I18N RESOURCES
// ----------------------------------------------------------------------------

const RESOURCES_EN = {};

const RESOURCES_ZH = {
    candlestick: {
        date: "交易日",
        ko: "开盘价",
        kh: "最高价",
        kl: "最低价",
        kc: "收盘价",
        ka: "成交额",
        kv: "成交量",
        kt: "换手率",
        ar: "日振幅",
        sr: "总振幅",
        ur: "涨跌幅",
    },
};

// ----------------------------------------------------------------------------
// COMPONENTS
// ----------------------------------------------------------------------------

i18n.use(initReactI18next).init({
    resources: {
        en: RESOURCES_EN,
        zh: RESOURCES_ZH,
    },
    lng: "zh",
    fallbackLng: "en",
    interpolation: { escapeValue: false },
});
