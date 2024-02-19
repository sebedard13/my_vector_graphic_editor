import { Component, ElementRef, AfterViewInit, ViewChild, HostListener } from "@angular/core";
import { animationFrames, map, merge, of, withLatestFrom } from "rxjs";
import { EventsService } from "src/app/events.service";
import { MouseInfoService } from "src/app/mouse-info/mouse-info.service";
import { ScenesService } from "src/app/scenes.service";
import { SelectionService } from "src/app/selection.service";
import { CanvasContent, ScreenCoord, draw, draw_closest_pt, render } from "wasm-vgc";

@Component({
    selector: "app-canvas",
    templateUrl: "./canvas.component.html",
    styleUrls: ["./canvas.component.scss"],
})
export class CanvasComponent implements AfterViewInit {
    @ViewChild("canvas") canvas!: ElementRef<HTMLCanvasElement>;
    private resizeObserver: ResizeObserver | undefined;
    private ctx!: CanvasRenderingContext2D;

    constructor(
        private mouseInfo: MouseInfoService,
        private scenesService: ScenesService,
        private selectionService: SelectionService,
        private eventService: EventsService,
    ) {}

    ngAfterViewInit(): void {
        const width = this.canvas.nativeElement.offsetWidth;
        const height = this.canvas.nativeElement.offsetHeight;
        this.canvas.nativeElement.width = width;
        this.canvas.nativeElement.height = height;

        this.scenesService.currentSceneChange$.subscribe(() => {
            this.scenesService.currentSceneNow((scene) => {
                scene.canvasContent.camera_set_pixel_region(
                    this.canvas.nativeElement.width,
                    this.canvas.nativeElement.height,
                );
            });
        });

        this.ctx = this.canvas.nativeElement.getContext("2d") as CanvasRenderingContext2D;

        this.resizeObserver = new ResizeObserver((_) => {
            this.hideCanvas();
            this.canvas.nativeElement.removeAttribute("width");
            this.canvas.nativeElement.removeAttribute("height");
            setTimeout(() => {
                this.canvas.nativeElement.width = this.canvas.nativeElement.offsetWidth;
                this.canvas.nativeElement.height = this.canvas.nativeElement.offsetHeight;

                this.scenesService.currentSceneNow((scene) => {
                    scene.canvasContent.camera_set_pixel_region(
                        this.canvas.nativeElement.width,
                        this.canvas.nativeElement.height,
                    );
                });
            }, 1);
        });

        this.resizeObserver.observe(this.canvas.nativeElement.parentElement!);

        animationFrames()
            .pipe(
                withLatestFrom(
                    merge(
                        of(null),
                        this.mouseInfo.mousePos$,
                        this.eventService.mouseLeave$.pipe(map(() => null)),
                    ),
                ),
            )
            .subscribe(([_, mouseInfo]) => {
                this.scenesService.currentSceneNow((scene) => {
                    this.render(scene.canvasContent, mouseInfo);
                });
            });
    }

    public render(canvasContent: CanvasContent, mouseCoords: { x: number; y: number } | null) {
        render(this.ctx, canvasContent);

        draw(this.selectionService.selection, canvasContent, this.ctx);
        if (mouseCoords != null) {
            draw_closest_pt(
                this.selectionService.selection,
                canvasContent,
                this.ctx,
                new ScreenCoord(mouseCoords.x, mouseCoords.y),
            );
        }

        this.showCanvas();
    }

    public hideCanvas() {
        this.canvas.nativeElement.style.opacity = "0";
    }

    public showCanvas() {
        this.canvas.nativeElement.style.opacity = "1";
    }

    @HostListener("mousemove", ["$event"])
    public onMouseMove(event: MouseEvent) {
        this.eventService.mouseMove.next(event);
    }

    @HostListener("wheel", ["$event"])
    public onMouseWheel(event: WheelEvent) {
        this.eventService.wheel.next(event);
    }

    @HostListener("mousedown", ["$event"])
    public onMouseDown(event: MouseEvent) {
        this.eventService.mouseDown.next(event);
    }

    @HostListener("mouseenter", ["$event"])
    public onMouseEnter(event: MouseEvent) {
        this.eventService.mouseEnter.next(event);
    }

    @HostListener("mouseleave", ["$event"])
    public onMouseLeave(event: MouseEvent) {
        this.eventService.mouseLeave.next(event);
    }
}
