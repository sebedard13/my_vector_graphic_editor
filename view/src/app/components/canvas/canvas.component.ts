import {
    OnInit,
    Component,
    ElementRef,
    AfterViewInit,
    ViewChild,
    Host,
    HostListener,
} from "@angular/core"
import { EventsService } from "src/app/events.service"
import { MouseInfoService } from "src/app/mouse-info/mouse-info.service"
import { ScenesService } from "src/app/scenes.service"
import { SelectionService } from "src/app/selection.service"
import {
    CanvasContent,
    Point,
    SelectedLevel,
    draw,
    draw_closest_pt,
    render,
} from "wasm-vgc"

@Component({
    selector: "app-canvas",
    templateUrl: "./canvas.component.html",
    styleUrls: ["./canvas.component.scss"],
})
export class CanvasComponent implements AfterViewInit {
    @ViewChild("canvas") canvas!: ElementRef<HTMLCanvasElement>
    private resizeObserver: ResizeObserver | undefined
    private ctx!: CanvasRenderingContext2D

    private canvasContent: CanvasContent | null = null

    private mouseCoords: { x: number; y: number } | null = null

    constructor(
        private mouseInfo: MouseInfoService,
        scenesService: ScenesService,
        private selectionService: SelectionService,
        private eventService: EventsService,
    ) {
        scenesService.currentScene$.subscribe((scene) => {
            this.canvasContent = scene
        })

        mouseInfo.mousePos.subscribe((coords) => {
            this.mouseCoords = coords
        })
    }

    ngAfterViewInit(): void {
        const width = this.canvas.nativeElement.offsetWidth
        const height = this.canvas.nativeElement.offsetHeight
        this.canvas.nativeElement.width = width
        this.canvas.nativeElement.height = height

        this.ctx = this.canvas.nativeElement.getContext(
            "2d",
        ) as CanvasRenderingContext2D

        this.resizeObserver = new ResizeObserver((entries) => {
            this.hideCanvas()
            this.canvas.nativeElement.removeAttribute("width")
            this.canvas.nativeElement.removeAttribute("height")
            setTimeout(() => {
                this.canvas.nativeElement.width =
                    this.canvas.nativeElement.offsetWidth
                this.canvas.nativeElement.height =
                    this.canvas.nativeElement.offsetHeight

                if (this.canvasContent == null) return
                this.canvasContent.set_pixel_region(
                    this.canvas.nativeElement.width,
                    this.canvas.nativeElement.height,
                )
            }, 1)
        })

        this.resizeObserver.observe(this.canvas.nativeElement.parentElement!)
        this.render()
    }

    public render() {
        if (this.canvasContent == null) return

        render(this.ctx, this.canvasContent)

        draw(this.selectionService.selection, this.canvasContent, this.ctx)
        if (this.mouseCoords != null) {
            draw_closest_pt(
                this.selectionService.selection,
                this.canvasContent,
                this.ctx,
                new Point(this.mouseCoords.x, this.mouseCoords.y),
            )
        }

        this.showCanvas()

        window.requestAnimationFrame(this.render.bind(this))
    }

    public hideCanvas() {
        this.canvas.nativeElement.style.opacity = "0"
    }

    public showCanvas() {
        this.canvas.nativeElement.style.opacity = "1"
    }

    @HostListener("mousemove", ["$event"])
    public onMouseMove(event: MouseEvent) {
        if (this.canvasContent == null) return

        this.mouseInfo.mousePos.next({ x: event.offsetX, y: event.offsetY })

        let pt = this.canvasContent.get_project_mouse(
            event.offsetX,
            event.offsetY,
        )
        this.mouseInfo.normalizedMousePos.next({ x: pt.x, y: pt.y })

        if (event.buttons == 4) {
            this.canvasContent.pan_camera(event.movementX, event.movementY)
        }

        //selection
        this.selectionService.selection.change_hover(this.canvasContent, pt)

        this.eventService.mouseMove.next(event)
    }

    @HostListener("wheel", ["$event"])
    public onMouseWheel(event: WheelEvent) {
        if (this.canvasContent == null) return

        this.canvasContent.zoom(event.deltaY * -1, event.offsetX, event.offsetY)
        this.mouseInfo.zoom.next(this.canvasContent.get_zoom())
        let pt = this.canvasContent.get_project_mouse(
            event.offsetX,
            event.offsetY,
        )
        this.mouseInfo.normalizedMousePos.next({ x: pt.x, y: pt.y })
    }

    @HostListener("mousedown", ["$event"])
    public onMouseDown(event: MouseEvent) {
        if (this.canvasContent == null) return

        if (event.buttons == 1) {
            if (event.shiftKey) {
                let point = this.canvasContent.get_project_mouse(
                    event.offsetX,
                    event.offsetY,
                )
                this.selectionService.selection.add_selection(
                    this.canvasContent,
                    point,
                )
                this.selectionService.selectionHasChanged.next()
            } else {
                let point = this.canvasContent.get_project_mouse(
                    event.offsetX,
                    event.offsetY,
                )
                this.selectionService.selection.change_selection(
                    this.canvasContent,
                    point,
                )
                this.selectionService.selectionHasChanged.next()
            }
        }
    }

    @HostListener("window:keydown.code.esc")
    public onEsc() {
        this.selectionService.selection.clear_to_level(SelectedLevel.None)
        this.selectionService.selectionHasChanged.next()
    }
}
