import {
	createSSRApp
} from "vue";
import App from "./App.vue";
// import uniDrawer from '@dcloudio/uni-ui/lib/uni-drawer/uni-drawer.vue';
export function createApp() {
	const app = createSSRApp(App);
	// app.component('uni-drawer', uniDrawer);
	return {
		app,
	};
}