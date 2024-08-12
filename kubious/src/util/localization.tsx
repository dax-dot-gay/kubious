import i18n from "i18next";
import { I18nextProvider, initReactI18next } from "react-i18next";

import * as langEn from "../lang/en.json";
import { ReactNode } from "react";

i18n.use(initReactI18next).init({
    fallbackLng: "en",
    debug: true,

    resources: {
        en: {
            translation: langEn,
        },
    },
});

export function LocalizationProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return <I18nextProvider i18n={i18n}>{children}</I18nextProvider>;
}
