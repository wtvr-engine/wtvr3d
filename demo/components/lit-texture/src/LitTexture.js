import { LitElement, html, css } from 'lit-element';
import {Scene,Vector3Data,FileType,LightType} from "../../../pkg/wtvr3d.js";

export class LitTexture extends LitElement {

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
        @media (min-width : 700px){
            .stats {
                position : absolute !important;
            }
        }
        canvas {
            height : 100%;
            width : 100%;
            display : block;
        }
        `;
    }
    firstUpdated() {
        this.init();
    }

    async getAssets(){
        let headTex = await this.getTexture("../../../assets/textures/HeadDiffuse.webp");
        let headMap = await this.getTexture("../../../assets/textures/HeadNormals.webp");
        let response = await fetch("../../../assets/meshes/head.wmesh");
        let mesh_data = new Uint8Array(await response.arrayBuffer());
        let response2 = await fetch("../../../assets/materials/normal_map.wmaterial");
        let material_data = new Uint8Array(await response2.arrayBuffer());
        let response3 = await fetch("../../../assets/materials/normal_map.wmatinstance");
        let mat_inst_data = new Uint8Array(await response3.arrayBuffer());
        return [mesh_data, material_data, mat_inst_data,headTex,headMap];
    }

    async init() {
        this.stats = new Stats();
        this.stats.showPanel( 0 ); 
        this.shadowRoot.appendChild( this.stats.dom );
        this.stats.dom.classList.add("stats");
        const [mesh, material, material_instance,texture,normal_map] = await this.getAssets();
        const canvas = this.shadowRoot.querySelector("canvas");
        const context = canvas.getContext("webgl");
        const scene = new Scene();
        const position = new Vector3Data(0,4,10);
        const towards = new Vector3Data(0,0,0);
        let camera_id = scene.create_camera_entity(16/9,3.14/4,1,1000, position,towards);
        scene.initialize(canvas, context, camera_id);
        scene.register_texture(texture,"head_diffuse");
        scene.register_texture(normal_map,"head_normal");
        let mesh_id = scene.register_asset(mesh,FileType.WMesh);
        let material_id = scene.register_asset(material,FileType.WMaterial);
        let matinstance_id = scene.register_asset(material_instance,FileType.WMatInstance);
        this.mesh_entity_id = scene.create_mesh_entity(mesh_id,matinstance_id);
        let ambiant = scene.create_light_entity(LightType.Ambiant,new Vector3Data(1.0,1.0,1.0),0.4,0.0,new Vector3Data(0.0,0.0,0.0));
        let dir = scene.create_light_entity(LightType.Directional,new Vector3Data(0.9,0.7,0.3),1.2,0.0,new Vector3Data(7,-4,-5));
        let dir2 = scene.create_light_entity(LightType.Directional,new Vector3Data(0.2,0.7,0.8),1.2,0.0,new Vector3Data(-10,-3.0,1));
        this.scene = scene;
        this.update_scene();
    }

    update_scene(){
        this.stats.begin();
        let deltaTime = performance.now() - this.time;
        this.time = performance.now();
        this.rotation = (this.rotation + (2*deltaTime * (3.14159*2) /12000 )) % (3.14159*2);
        const rotationVector = new Vector3Data(0.0,this.rotation,0.0);
        this.scene.set_transform_rotation(this.mesh_entity_id,rotationVector);
        this.scene.update();
        this.stats.end();
        this.animationFrameRequest = requestAnimationFrame(() => {
            this.update_scene();
        });
    }

    disconnectedCallback(){
        cancelAnimationFrame(this.animationFrameRequest);
        this.scene.free();
    }

    getTexture(filePath) {
        return new Promise((resolve,reject) => {
            const image = new Image();
            image.onload = () => {
                resolve(image);
            }
            image.src = filePath;
        });
    }

    render() {
        return html`<canvas></canvas>`;
    }
}