import { ReplaySubject, Subject, Subscription, filter, map, shareReplay } from "rxjs";
import { EventsService } from "../events.service";
import { ScenesService } from "../scenes.service";
import { Injectable, inject } from "@angular/core";
import { ScreenLength2d, ScreenCoord } from "wasm-vgc";
import { Functionality } from "./functionality";

@Injectable({
    providedIn: "root",
})
export class CameraService extends Functionality {
    private subscriptions: Subscription[] = [];
    private eventsService!: EventsService;
    private sceneService!: ScenesService;

    private zoomChange = new Subject<void>();
    private zoom = new ReplaySubject<number>(1);
    public zoom$ = this.zoom.asObservable();
    constructor() {
        super();

        this.eventsService = inject(EventsService);
        this.sceneService = inject(ScenesService);

        this.zoomChange
            .asObservable()
            .pipe(
                map((_) => {
                    const scene = this.sceneService.currentScene();
                    if (scene == null) {
                        return -1;
                    }
                    return scene.canvasContent.camera_get_zoom();
                }),
                shareReplay(1),
            )
            .subscribe((zoom) => {
                this.zoom.next(zoom);
            });

        this.sceneService.currentSceneChange$.subscribe((_) => {
            this.zoomChange.next();
        });
    }

    activate(): void {
        const zoomEvent = this.eventsService.wheel$.subscribe((event) => {
            this.sceneService.currentSceneNow((scene) => {
                scene.canvasContent.camera_zoom_at(
                    event.deltaY * -1,
                    new ScreenCoord(event.offsetX, event.offsetY),
                );
                this.zoomChange.next();
            });
        });
        this.subscriptions.push(zoomEvent);

        const moveEvent = this.eventsService.mouseMove$
            .pipe(filter((event) => event.buttons == 4 || (event.shiftKey && event.buttons == 1)))
            .subscribe((event) => {
                this.sceneService.currentSceneNow((scene) => {
                    scene.canvasContent.camera_pan_by(
                        new ScreenLength2d(event.movementX, event.movementY),
                    );
                });
            });
        this.subscriptions.push(moveEvent);

        const rotate5 = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "q"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    let angle = scene.canvasContent.camera_get_rotation();
                    angle += 0.0872664626;
                    scene.canvasContent.camera_set_rotation(angle);
                });
            });
        this.subscriptions.push(rotate5);

        const rotateHome = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "r"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    scene.canvasContent.camera_set_rotation(0);
                });
            });
        this.subscriptions.push(rotateHome);

        const rotateMinus5 = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "e"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    let angle = scene.canvasContent.camera_get_rotation();
                    angle -= 0.0872664626;
                    scene.canvasContent.camera_set_rotation(angle);
                });
            });
        this.subscriptions.push(rotateMinus5);

        const flipX = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "z"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    const flip = scene.canvasContent.camera_get_reflect_x();
                    scene.canvasContent.camera_set_reflect_x(!flip);
                });
            });
        this.subscriptions.push(flipX);

        const flipY = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "x"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    const flip = scene.canvasContent.camera_get_reflect_y();
                    scene.canvasContent.camera_set_reflect_y(!flip);
                });
            });
        this.subscriptions.push(flipY);

        const reset = this.eventsService.keydown$
            .pipe(filter((event) => event.key == "c"))
            .subscribe((_) => {
                this.sceneService.currentSceneNow((scene) => {
                    scene.canvasContent.camera_home();
                });
            });
        this.subscriptions.push(reset);
    }

    desactivate(): void {
        this.subscriptions.forEach((subscription) => subscription.unsubscribe());
        this.subscriptions = [];
    }

    isActivated(): boolean {
        return this.subscriptions.length > 0;
    }
}
