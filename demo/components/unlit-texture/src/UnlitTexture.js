import { LitElement, html, css } from 'lit-element';
import {Scene,Vector3Data,FileType} from "../../../pkg/wtvr3d.js";

export class UnlitTexture extends LitElement {

    constructor(){
        super();
        this.time = performance.now();
        this.rotation = 0;
    }
    static get styles() {
        return css`
        :host {
            position : relative;
            display : block;
            height : 100%;
            width : 100%;
        }
        canvas {
            height : 100%;
            width : 100%;
        }
        `;
    }
    firstUpdated() {
        this.init();
    }

    async getAssets(){
        let response = await fetch("../../../assets/meshes/test_monkey-0.wmesh");
        let mesh_data = new Uint8Array(await response.arrayBuffer());
        let response2 = await fetch("../../../assets/materials/test_uniform_color.wmaterial");
        let material_data = new Uint8Array(await response2.arrayBuffer());
        let response3 = await fetch("../../../assets/materials/test_with_override.wmatinstance");
        let mat_inst_data = new Uint8Array(await response3.arrayBuffer());
        return [mesh_data, material_data, mat_inst_data];
    }

    async init() {
        const [mesh, material, material_instance] = await this.getAssets();
        const canvas = this.shadowRoot.querySelector("canvas");
        const context = canvas.getContext("webgl");
        const scene = new Scene();
        const position = new Vector3Data(0,4,10);
        const towards = new Vector3Data(0,0,0);
        let camera_id = scene.create_camera_entity(16/9,3.14/4,1,1000, position,towards);
        scene.initialize(canvas, context, camera_id);
        let mesh_id = scene.register_asset(mesh,FileType.WMesh);
        let material_id = scene.register_asset(material,FileType.WMaterial);
        let matinstance_id = scene.register_asset(material_instance,FileType.WMatInstance);
        this.mesh_entity_id = scene.create_mesh_entity(mesh_id,matinstance_id);
        this.scene = scene;
        this.update_scene();
    }

    update_scene(){
        let deltaTime = performance.now() - this.time;
        this.time = performance.now();
        this.rotation = (this.rotation + (2*deltaTime * (3.14159*2) /12000 )) % (3.14159*2);
        const rotationVector = new Vector3Data(0.0,this.rotation,0.0);
        this.scene.set_transform_rotation(this.mesh_entity_id,rotationVector);
        this.scene.update();
        this.animationFrameRequest = requestAnimationFrame(() => {
            this.update_scene();
        });
    }

    disconnectedCallback(){
        cancelAnimationFrame(this.animationFrameRequest);
        this.scene.free();
    }

    render() {
        return html`<canvas></canvas>`;
    }
}