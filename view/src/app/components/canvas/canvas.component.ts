import { Component, ElementRef, AfterViewInit, ViewChild, HostListener } from "@angular/core";
import { EventsService } from "src/app/events.service";
import { MouseInfoService } from "src/app/mouse-info/mouse-info.service";
import { ScenesService } from "src/app/scenes.service";
import { SelectionService } from "src/app/selection.service";
import { CanvasContent, Point, draw, draw_closest_pt, render } from "wasm-vgc";

@Component({
    selector: "app-canvas",
    templateUrl: "./canvas.component.html",
    styleUrls: ["./canvas.component.scss"],
})
export class CanvasComponent implements AfterViewInit {
    @ViewChild("canvas") canvas!: ElementRef<HTMLCanvasElement>;
    private resizeObserver: ResizeObserver | undefined;
    private ctx!: CanvasRenderingContext2D;

    private canvasContent: CanvasContent | null = null;

    private mouseCoords: { x: number; y: number } | null = null;

    constructor(
        mouseInfo: MouseInfoService,
        scenesService: ScenesService,
        private selectionService: SelectionService,
        private eventService: EventsService,
    ) {
        scenesService.currentScene$.subscribe((scene) => {
            this.canvasContent = scene;
            this.canvasContent.set_pixel_region(
                this.canvas.nativeElement.width,
                this.canvas.nativeElement.height,
            );
        });

        mouseInfo.mousePos$.subscribe((coords) => {
            this.mouseCoords = coords;
        });
    }

    ngAfterViewInit(): void {
        const width = this.canvas.nativeElement.offsetWidth;
        const height = this.canvas.nativeElement.offsetHeight;
        this.canvas.nativeElement.width = width;
        this.canvas.nativeElement.height = height;

        this.ctx = this.canvas.nativeElement.getContext("2d") as CanvasRenderingContext2D;

        this.resizeObserver = new ResizeObserver((_) => {
            this.hideCanvas();
            this.canvas.nativeElement.removeAttribute("width");
            this.canvas.nativeElement.removeAttribute("height");
            setTimeout(() => {
                this.canvas.nativeElement.width = this.canvas.nativeElement.offsetWidth;
                this.canvas.nativeElement.height = this.canvas.nativeElement.offsetHeight;

                if (this.canvasContent == null) return;
                this.canvasContent.set_pixel_region(
                    this.canvas.nativeElement.width,
                    this.canvas.nativeElement.height,
                );
            }, 1);
        });

        this.resizeObserver.observe(this.canvas.nativeElement.parentElement!);
        this.render();
    }

    public render() {
        if (this.canvasContent == null) return;

        console.log("render" + this.canvasContent.get_uuid());

        render(this.ctx, this.canvasContent);

        draw(this.selectionService.selection, this.canvasContent, this.ctx);
        if (this.mouseCoords != null) {
            draw_closest_pt(
                this.selectionService.selection,
                this.canvasContent,
                this.ctx,
                new Point(this.mouseCoords.x, this.mouseCoords.y),
            );
        }

        this.showCanvas();

        window.requestAnimationFrame(this.render.bind(this));
    }

    public hideCanvas() {
        this.canvas.nativeElement.style.opacity = "0";
    }

    public showCanvas() {
        this.canvas.nativeElement.style.opacity = "1";
    }

    @HostListener("mousemove", ["$event"])
    public onMouseMove(event: MouseEvent) {
        if (this.canvasContent == null) return;

        this.eventService.mouseMove.next(event);
    }

    @HostListener("wheel", ["$event"])
    public onMouseWheel(event: WheelEvent) {
        if (this.canvasContent == null) return;

        this.eventService.wheel.next(event);
    }

    @HostListener("mousedown", ["$event"])
    public onMouseDown(event: MouseEvent) {
        if (this.canvasContent == null) return;

        this.eventService.mouseDown.next(event);
    }
}
