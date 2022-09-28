import { cloneElement, isValidElement, ReactNode, useRef } from "react";
import noop from "../../utils/noop";

interface Props<Value> {
  value?: Value;
  valueProps?: string;
  onChangeProps?: string;
  waitTime?: number;
  onChange?: (value: Value) => void;
  onFormat?: (...args: any[]) => Value;
  onGuard?: (value: Value, oldValue: Value) => Promise<void>;
  onCatch?: (error: Error) => void;
  children: ReactNode;
}

function GuardState<T>(props: Props<T>) {
  const {
    value,
    children,
    valueProps = "value",
    onChangeProps = "onChange",
    waitTime = 0, // debounce wait time default 0
    onGuard = noop,
    onCatch = noop,
    onChange = noop,
    onFormat = (v: T) => v,
  } = props;

  const lockRef = useRef(false);
  const saveRef = useRef(value);
  const lastRef = useRef(0);
  const timeRef = useRef<any>();

  if (!isValidElement(children)) {
    return children as any;
  }

  const childProps = { ...children.props };

  childProps[valueProps] = value;
  childProps[onChangeProps] = async (...args: any[]) => {
    if (lockRef.current) return;

    lockRef.current = true;

    try {
      const newValue = (onFormat as any)(...args);
      onChange(newValue);

      const now = Date.now();

      if (waitTime <= 0 || now - lastRef.current >= waitTime) {
        saveRef.current = value;
      }

      lastRef.current = now;

      if (waitTime <= 0) {
        await onGuard(newValue, value!);
      } else {
        clearTimeout(timeRef.current);

        timeRef.current = setTimeout(async () => {
          try {
            await onGuard(newValue, saveRef.current!);
          } catch (err: any) {
            onChange(saveRef.current!);
            onCatch(err);
          }
        }, waitTime);
      }
    } catch (err: any) {
      onChange(saveRef.current!);
      onCatch(err);
    }
    lockRef.current = false;
  };
  return cloneElement(children, childProps);
}

export default GuardState;
