import {
    Coord as CoordCl,
    ScreenCoord as ScreenCoordCl,
    ScreenLength2d as ScreenCoord2dCl,
    ScreenRect as ScreenRectCl,
    Rgba as RgbaCl,
} from "wasm-client";

export class Coord implements CoordCl {
    public c: { x: number; y: number };

    public get x(): number {
        return this.c.x;
    }

    public get y(): number {
        return this.c.y;
    }

    public set x(x: number) {
        this.c.x = x;
    }

    public set y(y: number) {
        this.c.y = y;
    }

    constructor(x: number, y: number) {
        this.c = { x, y };
    }
}

export class ScreenCoord extends Coord implements ScreenCoordCl {
    constructor(x: number, y: number) {
        super(x, y);
    }
}

export class ScreenLength2d extends Coord implements ScreenCoord2dCl {
    constructor(x: number, y: number) {
        super(x, y);
    }
}

export class ScreenRect implements ScreenRectCl {
    public top_left: ScreenCoord;
    public bottom_right: ScreenCoord;

    constructor(screenRectCl: ScreenRectCl) {
        this.top_left = new ScreenCoord(screenRectCl.top_left.c.x, screenRectCl.top_left.c.y);
        this.bottom_right = new ScreenCoord(
            screenRectCl.bottom_right.c.x,
            screenRectCl.bottom_right.c.y,
        );
    }

    public width(): number {
        return this.bottom_right.x - this.top_left.x;
    }

    public height(): number {
        return this.bottom_right.y - this.top_left.y;
    }
}

export class Rgba implements RgbaCl {
    // interger values from 0 to 255
    public r: number;
    public g: number;
    public b: number;
    public a: number;

    constructor(r: number, g: number, b: number, a: number) {
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }

    public static fromCSSHex(str: string): Rgba {
        const r = parseInt(str.substring(1, 3), 16);
        const g = parseInt(str.substring(3, 5), 16);
        const b = parseInt(str.substring(5, 7), 16);

        return new Rgba(r, g, b, 255);
    }

    public toCSSHex(): string {
        return `#${this.r.toString(16).padStart(2, "0")}${this.g.toString(16).padStart(2, "0")}${this.b.toString(16).padStart(2, "0")}`;
    }
}
