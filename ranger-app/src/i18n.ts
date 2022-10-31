import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';

import Backend from 'i18next-http-backend';
import LanguageDetector from 'i18next-browser-languagedetector';

// eslint-disable-next-line @typescript-eslint/no-floating-promises
i18n // eslint-disable-line import/no-named-as-default-member
  .use(Backend)
  .use(LanguageDetector)
  .use(initReactI18next).init({
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false,
    },
  });

export {default} from 'i18next';
