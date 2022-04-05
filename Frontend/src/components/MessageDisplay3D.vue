<template>
  <div id="cyber"></div>
</template>

<script>
import { toRaw } from "vue";
import { onMounted } from "vue";
import * as THREE from "three";
import { FontLoader } from "three/examples/jsm/loaders/FontLoader.js";
import { TextGeometry } from "three/examples/jsm/geometries/TextGeometry.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";

export default {
  name: "MessageDisplay3D",
  props: ["width", "height"],
  setup() {
    let scene = new THREE.Scene();

    const generateBox = (w, h, d) => {
      let geo = new THREE.BoxGeometry(w, h, d);
      let tex = new THREE.TextureLoader().load("/texture/moon.jpg");
      let mat = new THREE.MeshLambertMaterial({
        map: tex,
      });

      let mesh = new THREE.Mesh(geo, mat);
      mesh.castShadow = true;
      return mesh;
    };

    const update = (renderer, scene, camera, controls) => {
      renderer.render(scene, camera);
      let moon = scene.getObjectByName("moon");
      let earth = scene.getObjectByName("earth");
      //sphere.position.z += 0.002;
      moon.rotation.y += 0.002;
      earth.rotation.y += 0.001;
      //box.rotation.x += 0.005;
      //box.rotation.y += 0.005;

      //scene.traverse(function (child) {});
      controls.update();
      requestAnimationFrame(() => {
        update(renderer, scene, camera, controls);
      });
    };

    // add background
    //this.scene.background = new THREE.CubeTextureLoader()
    //.setPath("/texture/space/")
    //.load(["px.jpg", "nx.jpg", "py.jpg", "ny.jpg", "pz.jpg", "nz.jpg"]);

    let pointLight = new THREE.PointLight(0xffffff, 1);
    pointLight.penumbra = 0.5;
    pointLight.castShadow = true;
    pointLight.shadow.mapSize.width = 2048;
    pointLight.shadow.mapSize.height = 2048;
    pointLight.shadow.bias = 0.001;
    pointLight.position.set(0, 5, 5);

    scene.add(pointLight);
    let box = generateBox(1, 1, 1);
    box.name = "box";
    box.position.z = -5;

    //scene.add(box);

    //moon
    const geometry = new THREE.SphereGeometry(7, 32, 32);
    const moonTexture = new THREE.TextureLoader().load("/texture/moon.jpg");
    const moonMat = new THREE.MeshLambertMaterial({
      map: moonTexture,
      color: 0xff5000,
    });

    const sphere = new THREE.Mesh(geometry, moonMat);
    sphere.castShadow = true;
    sphere.name = "moon";
    sphere.position.y = -5;
    sphere.position.z = -10;

    let loader = new FontLoader();
    loader.load("/fonts/Demo_Regular.json", (font) => {
      let textGeo = new TextGeometry("Hi dude How are you today?!", {
        font: font,
        curveSegments: 12,

        bevelThickness: 2,
        bevelSize: 5,
        bevelEnabled: true,
      });

      let textMaterial = new THREE.MeshPhongMaterial({
        color: "rgb(100,100,100)",
        side: THREE.DoubleSide,
      });

      let mesh = new THREE.Mesh(textGeo, textMaterial);
      mesh.position.set(0, 0, -15);
      mesh.scale.set(0.001, 0.001, 0.001);
      //mesh.rotateY(Math.PI / 5)

      sphere.add(mesh);
    });

    scene.add(sphere);

    //earth
    const geometry2 = new THREE.SphereGeometry(7, 32, 32);
    const moonTexture2 = new THREE.TextureLoader().load("/texture/earth.jpg");
    const moonMat2 = new THREE.MeshLambertMaterial({ map: moonTexture2 });

    const sphere2 = new THREE.Mesh(geometry2, moonMat2);
    sphere2.receiveShadow = true;
    sphere2.name = "earth";
    sphere2.position.x = 4;
    sphere2.position.y = 7;
    sphere2.position.z = -60;

    scene.add(sphere2);
    scene.add(new THREE.AmbientLight(0xffffff, 0.3));

    onMounted(() => {
      // get message-display size
      const messageDisplayElement = document.getElementById("messages-display");

      // create renderer
      let renderer = new THREE.WebGLRenderer();
      renderer.shadowMap.enabled = true;
      renderer.setSize(
        messageDisplayElement.clientWidth,
        messageDisplayElement.clientHeight
      );
    
      //create camera    
      let camera = new THREE.PerspectiveCamera(
        45,
        messageDisplayElement.clientWidth / messageDisplayElement.clientHeight,
        1,
        1000
      );
      camera.position.set(0, 0, 8);
      camera.lookAt(new THREE.Vector3(0, 0, 0));

      // create controls
      let controls = new OrbitControls(camera, renderer.domElement);

      //mount
      document.getElementById("cyber").appendChild(renderer.domElement);
      update(renderer, scene, camera, controls);
    });
  },
};
</script>

<style>
</style>