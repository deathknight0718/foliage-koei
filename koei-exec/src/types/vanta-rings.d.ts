declare module "vanta/dist/vanta.rings.min" {
    interface VantaEffect {
        destroy(): void;
        resize(): void;
        setOptions(options: Partial<VantaRingsOptions>): void;
    }

    interface VantaRingsOptions {
        el: HTMLElement | string;
        THREE: typeof import("three");
        mouseControls?: boolean;
        touchControls?: boolean;
        gyroControls?: boolean;
        minHeight?: number;
        minWidth?: number;
        scale?: number;
        scaleMobile?: number;
        backgroundColor?: number;
        color?: number;
        backgroundAlpha?: number;
        maxDistance?: number;
        spacing?: number;
        showDots?: boolean;
    }

    function RINGS(options: VantaRingsOptions): VantaEffect;

    interface VantaRingsModule {
        default: typeof RINGS;
    }

    const vantaRings: VantaRingsModule;

    export default vantaRings;
}