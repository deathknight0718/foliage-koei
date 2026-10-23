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
import Button from "@mui/material/Button";
import type { ReactNode } from "react";
import { useEffect, useRef } from "react";
import * as THREE from "three";
import rings from "vanta/dist/vanta.rings.min";
import Layout from "../components/Layout";
import { useRouter } from "../utils/route";

// ----------------------------------------------------------------------------

function useBackground() {
    const ref = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (!ref.current) return;
        const effect = rings.default({
            el: ref.current,
            THREE,
            mouseControls: true,
            touchControls: true,
            gyroControls: false,
            minHeight: 200.00,
            minWidth: 200.00,
            scale: 1.00,
            scaleMobile: 1.00,
            backgroundColor: 0x0,
            color: 0xff0000,
        });
        return () => effect.destroy();
    }, []);
    return ref;
}

function ButtonRouter({ label, route }: { label: string; route: string }) {
    const { push } = useRouter();
    return (
        <Button size="large" variant="contained" onClick={() => push(route)}>
            {label}
        </Button>
    );
}

// ----------------------------------------------------------------------------

function Home() {
    const ref = useBackground();
    return (
        <Layout className="flex flex-col min-h-screen">
            <img src="/logo.svg" alt="Foliage Logo" width={131} height={32} className="fixed top-6 left-6" />
            <div className="fixed top-1/2 left-16 -translate-y-7/8">
                <h1 className="text-h3 lg:text-h1">Foliage Koei</h1>
                <p className="text-h5 lg:text-h4 mt-4">光荣系列游戏数据修改工具</p>
                <div className="flex items-center gap-4 mt-16">
                    <ButtonRouter label="共享数据" route="/fixdata" />
                </div>
            </div>
            <div ref={ref} className="fixed top-0 left-0 w-full h-full -z-10" />
        </Layout>
    );
}

export default Home;
