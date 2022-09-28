import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "../locales/en.json";
import pt from "../locales/pt.json";

const resources = {
  en: { translation: en },
  pt: { translation: pt },
};

i18n.use(initReactI18next).init({
  resources,
  lng: "en",
  interpolation: {
    escapeValue: false,
  },
});
