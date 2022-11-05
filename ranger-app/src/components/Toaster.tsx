import {Intent, Position, Toaster} from '@blueprintjs/core';

export const AppToaster = Toaster.create({
  className: 'recipe-toaster',
  position: Position.TOP,
  // eslint-disable-next-line unicorn/prefer-query-selector
}, document.getElementById('toast') ?? undefined);

export const toastSuccess = (message: string) => (
  AppToaster.show({
    icon: 'tick',
    intent: Intent.SUCCESS,
    message: `${message}`,
  })
);

export const toastWarning = (message: string) => (
  AppToaster.show({
    icon: 'warning-sign',
    intent: Intent.DANGER,
    message: `${message}`,
  })
);
