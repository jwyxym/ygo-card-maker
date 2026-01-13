import { defineConfig } from 'vite'
import uni from '@dcloudio/vite-plugin-uni'
import vueSetupExtend from 'vite-plugin-vue-setup-extend'
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        uni(),
        vueSetupExtend()
    ],
})