export abstract class Functionality {
    abstract activate(): void;
    abstract desactivate(): void;
    abstract isActivated(): boolean;
    toggle(): void {
        if (this.isActivated()) {
            this.desactivate();
        } else {
            this.activate();
        }
    }
}
