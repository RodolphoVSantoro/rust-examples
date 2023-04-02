type Sync<T> = Exclude<T, Promise<any>>

type KeyType = string | number
type KeyFunction<T> = (object: Sync<T>)=>KeyType
type ObjectMap<T> = Record<KeyType, Sync<T>>
type Agrupator<T> = KeyType | KeyFunction<T>

function groupSumBy
	<T, B = Agrupator<T>>
	(
		toGroup: T[],
		groupBy: B,
		summer: ((object: T, result: ObjectMap<T>, fn: B)=>void)
	): ObjectMap<T> {
		const result: ObjectMap<T> = {}
		toGroup.forEach(groupee=>summer(groupee, result, groupBy))
		return result
}

function isOdd(n: number) {
	return n%2 !== 0 ? 1 : 0
}

function isEven(n: number) {
	return n%2 !== 0 ? 0 : 1
}

function sumNumberByGroup<T>(n: number, result: ObjectMap<number>, group: Agrupator<T>) {
	if(result[group(n)] === undefined) {
		result[group(n)] = 0
	}
	result[group(n)] += n
}

async function f() {
	return [1,2,3]
}

function test<T>(t: Sync<T>) {
	return t
}

async function main(){
	const a = f()
	test(a)
	test(await a)
	console.log(groupSumBy(await f(), isOdd, sumNumberByGroup))
	console.log(groupSumBy(await f(), isEven, sumNumberByGroup))
}
main().then()
