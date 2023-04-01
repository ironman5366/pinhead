import {MantineThemeOverride} from "@mantine/core";

const Theme: MantineThemeOverride = {
    colorScheme: "light",
    colors: {
        cerulean: ['#AFFFFA', '#A3FFE0', '#97FED6', '#8BF4CC', '#7FDBC2', '#73C2B8', '#67A9AE', '#5B90A4', '#4F869A', '#437C90'],
        slate: ['#CAF2F0', '#B8E1DF', '#A6D0CE', '#94BFBD', '#82AEAC', '#709D9B', '#5E8C8A', '#4C7B79', '#3A6A68', '#255957'],
        goldenrod: ['#FFF4CA', '#FFEBB6', '#FFE2A2', '#FFD98E', '#FFD07A', '#FFC766', '#FFBE52', '#FFB53E', '#FFAC2A', '#A98743'],
        saffron: ['#FFFFFF', '#FFF7E0', '#FFF0C2', '#FFE9A4', '#FFE286', '#FFDB68', '#FFD44A', '#FFCD2C', '#FFC60E', '#F7C548']
    },
    primaryColor: "cerulean",
    fontFamily: "Patua One",
}

export default Theme;
