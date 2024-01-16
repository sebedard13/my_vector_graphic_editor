import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "../scenes.service";
import { ModalService } from "../modal.service";
import { NewSceneComponent } from "../new-scene/new-scene.component";

@Component({
    selector: "app-save-load",
    templateUrl: "./save-load.component.html",
    styleUrl: "./save-load.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SaveLoadComponent {
    constructor(
        protected scenesService: ScenesService,
        private modalService: ModalService,
    ) {}

    newScene() {
        this.modalService.showModal(NewSceneComponent);
    }
}
