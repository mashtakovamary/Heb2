import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('LogIN'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(7)

WebUI.click(findTestObject('object2/a_Fruit  Vegetables'))

WebUI.click(findTestObject('object2/Cabbage and Greens'))

WebUI.click(findTestObject('Fresh Brussels Sprouts'))

WebUI.click(findTestObject('Save For Later'))

WebUI.delay(7)

WebUI.click(findTestObject('Edit'))

WebUI.click(findTestObject('object2/Save For later List'))

WebUI.getElementLeftPosition(findTestObject('object2/img'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementLeftPosition(findTestObject('object2/Shop Assistant'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementWidth(findTestObject('object2/p_Test Banner 1'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementHeight(findTestObject('object2/a_View Weekly Ad'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementHeight(findTestObject('object2/a_Yellow Coupons'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementHeight(findTestObject('object2/li_test'), FailureHandling.STOP_ON_FAILURE)

WebUI.getElementHeight(findTestObject('object2/div_Fruit  VegetablesMeat and'), FailureHandling.STOP_ON_FAILURE)

