export const spyOn = (obj, method, returnValue) => {
    const stats = {
        callCount: 0,
        called: false,
        calls: [],
        returnValue: null,
    };

    Object.defineProperty(obj, method, {
        value: (...args) => {
            stats.callCount++;
            stats.called = true;
            stats.calls.push(...args);
            stats.returnValue = returnValue();
            return stats.returnValue;
        },
        writable: true,
    });

    return stats;
};
