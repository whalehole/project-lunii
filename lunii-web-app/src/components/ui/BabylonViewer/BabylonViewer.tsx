'use client'

import {Engine, Scene, Vector3, HemisphericLight, ImportMeshAsync, ArcRotateCamera} from "@babylonjs/core";
import { registerBuiltInLoaders } from "@babylonjs/loaders/dynamic";
import { useEffect, useRef } from "react";
import styles from "./BabylonViewer.module.css";

export default function BabylonViewer() {
    registerBuiltInLoaders();

    const reactCanvas = useRef(null);

    useEffect(() => {
        const { current: canvas } = reactCanvas;

        if (!canvas) return;

        const createScene = () => {
            return new Scene(engine);
        }

        const engine = new Engine(canvas);
        const scene = createScene();

        const camera = new ArcRotateCamera(
            "camera1",
            Math.PI / 2,
            Math.PI / 2,
            0.5,
            new Vector3(0, 1.2, 0),
        )
        camera.attachControl(canvas, true);
        camera.minZ = 0.01;

        const light = new HemisphericLight("light", new Vector3(0, 1, 0), scene);
        light.intensity = 0.7;

        fetch('http://localhost:55000/file/entity')
            .then(res => res.text())
            .then( url => {
                return ImportMeshAsync(url, scene);
        })
            .then();

        // ImportMeshAsync(
        //     "http://localhost:55000/file/entity",
        //     scene,
        // ).then();

        engine.runRenderLoop(() => {
            scene.render();
        })

    }, []);

    return <canvas className={styles.viewerCanvas} ref={reactCanvas} />;
}