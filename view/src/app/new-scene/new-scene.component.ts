import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ModalComponent } from "../modal.service";
import { Observable, Subject } from "rxjs";

@Component({
    selector: "app-new-scene",
    templateUrl: "./new-scene.component.html",
    styleUrl: "./new-scene.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class NewSceneComponent implements ModalComponent {
    public private = new Subject<unknown>();

    protected width = 1280;
    protected height = 720;
    protected ratio = (1280 / 720).toFixed(3);
    protected name = "New Scene";

    closeModal(): Observable<unknown> {
        return this.private.asObservable();
    }
}
